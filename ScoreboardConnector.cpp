#include "ScoreboardConnector.h"
#include "Logger.hpp"
#include "Exceptions.h"
#include <nlohmann/json.hpp>

#include <utility>
#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>
#include <format>
#include <functional>
#include <thread>

using namespace derby_stats;
using namespace std;
using namespace std::placeholders;

using json = nlohmann::json;

websocketpp::log::level get_socket_level_for_logging_level(LOG_LEVEL log_level);

unique_ptr<ScoreboardConnector> ScoreboardConnector::create()
{
	return std::make_unique<DisconnectedScoreboardConnector>();
}

void ScoreboardConnector::set_state_update_handler(state_update_handler handler)
{
	this->update_handler = move(handler);
}

shared_ptr<ScoreboardConnector> DisconnectedScoreboardConnector::connect(const string url)
{
	Logger::log_info("Connecting to websocket endpoint {}", url);

	return std::shared_ptr<ConnectedScoreboardConnector>(new ConnectedScoreboardConnector(url));
}

bool DisconnectedScoreboardConnector::get_is_open()
{
	return false;
}

void DisconnectedScoreboardConnector::register_topic(string)
{
	throw NotConnectedException();
}

shared_ptr<ScoreboardConnector> ConnectedScoreboardConnector::connect(string)
{
	throw AlreadyConnectedException();
}

ConnectedScoreboardConnector::ConnectedScoreboardConnector(string url)
{
	this->socket = std::make_unique<websocket_client>();

	this->socket->clear_access_channels(websocketpp::log::alevel::all);
	this->socket->set_access_channels(get_socket_level_for_logging_level(Logger::get_log_level()));

	this->socket->init_asio();

	this->socket->set_message_handler([this](auto&& handle, auto&& message) { this->handle_message(handle, message); });
	this->socket->set_open_handler([this](auto&& handle) { this->handle_open(handle); });

	auto const websocket_url = format("ws://{}/WS", url);

	error_code error_code;
	this->connection = this->socket->get_connection(websocket_url, error_code);

	this->socket->connect(this->connection);

	this->run_thread = thread([this]()
		{
			this->socket->run();
		});

	while (!this->is_open)
	{
		this_thread::sleep_for(chrono::milliseconds(10));
	}
}

void ConnectedScoreboardConnector::handle_message(websocketpp::connection_hdl, const message_ptr& message)
{
	auto const payload = message->get_payload();

	Logger::log_trace("Message received: {}", payload);

	if (this->update_handler == nullptr)
	{
		Logger::log_debug("No message handler specified. Message ignored.");
		return;
	}

	auto const message_body = json::parse(payload);

	for(auto& item: message_body["state"].items())
	{
		auto const key = item.key();
		auto const value = item.value().dump();

		Logger::log_debug("Sending update");
		Logger::log_trace(R"(Sending update for key/value pair: "{}": "{}")", key, value);

		update_handler(key, value);
	}
}

void ConnectedScoreboardConnector::handle_open(const websocketpp::connection_hdl)
{
	this->is_open = true;
}

bool ConnectedScoreboardConnector::get_is_open()
{
	return this->is_open;
}

void ConnectedScoreboardConnector::register_topic(string topic_name)
{
	Logger::log_debug("Registering topic '{}'", topic_name);

	try
	{
		auto const data = format(R"({{"action":"Register","paths":["{}"]}})", topic_name);
		this->socket->send(this->connection, data.c_str(), data.length(), websocketpp::frame::opcode::TEXT);
	}
	catch (websocketpp::exception const& ex)
	{
		Logger::log_error("Exception opening websocket connection: {}", ex.what());
	}
}

websocketpp::log::level get_socket_level_for_logging_level(const LOG_LEVEL log_level)
{
	switch (log_level)
	{
	case LOG_LEVEL_FATAL:
	case LOG_LEVEL_ERROR:
	case LOG_LEVEL_WARN:
		return websocketpp::log::alevel::fail;

	case LOG_LEVEL_INFO:
		return
			websocketpp::log::alevel::fail
			| websocketpp::log::alevel::connect
			| websocketpp::log::alevel::disconnect;

	case LOG_LEVEL_DEBUG:
		return
			websocketpp::log::alevel::fail
			| websocketpp::log::alevel::connect
			| websocketpp::log::alevel::disconnect
			| websocketpp::log::alevel::debug_close
			| websocketpp::log::alevel::debug_handshake;

	case LOG_LEVEL_TRACE:
		return websocketpp::log::alevel::all;
	}

	return websocketpp::log::alevel::none;
}
