#include "ScoreboardConnector.h"
#include "Logger.hpp"
#include "Exceptions.h"

#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>
#include <functional>
#include <thread>

using namespace derby_stats;
using namespace std;
using namespace std::placeholders;

unique_ptr<ScoreboardConnector> ScoreboardConnector::create()
{
	return std::make_unique<DisconnectedScoreboardConnector>();
}


unique_ptr<ScoreboardConnector> DisconnectedScoreboardConnector::connect(string url)
{
	Logger::log_info("Connecting to websocket endpoint {}", url);

	return std::unique_ptr<ConnectedScoreboardConnector>(new ConnectedScoreboardConnector(url));
}

unique_ptr<ScoreboardConnector> ConnectedScoreboardConnector::connect(string url)
{
	throw AlreadyConnectedException();
}

websocketpp::log::level get_socket_level_for_logging_level(LOG_LEVEL log_level)
{
	switch(log_level)
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

	default:
		return websocketpp::log::alevel::none;
	}
}

ConnectedScoreboardConnector::ConnectedScoreboardConnector(string url)
{
	this->socket = std::make_unique<websocket_client>();
	this->socket->clear_access_channels(websocketpp::log::alevel::all);

	this->socket->set_access_channels(get_socket_level_for_logging_level(Logger::get_log_level()));

	this->socket->init_asio();

	this->socket->set_message_handler([](websocketpp::connection_hdl handle, message_ptr message)
		{
			cout << message->get_payload() << endl;
		});

	this->socket->set_open_handler([this](websocketpp::connection_hdl connection_handle)
		{
			auto const data = "{\"action\":\"Register\",\"paths\":[\"ScoreBoard.CurrentGame.UpcomingJam\"]}";
			try
			{
				this->socket->send(connection_handle, data, strlen(data), websocketpp::frame::opcode::TEXT);
			}
			catch (websocketpp::exception const& ex)
			{
				Logger::log_error("Exception opening websocket connection: {}", ex.what());
			}
		});

	websocketpp::lib::error_code error_code;
	auto const connection = this->socket->get_connection(url, error_code);

	this->socket->connect(connection);

	this->run_thread = thread([this]()
		{
			this->socket->run();
		});
}
