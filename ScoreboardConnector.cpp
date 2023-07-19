#include "ScoreboardConnector.h"
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
	return std::unique_ptr<ConnectedScoreboardConnector>(new ConnectedScoreboardConnector(url));
}

unique_ptr<ScoreboardConnector> ConnectedScoreboardConnector::connect(string url)
{
	throw AlreadyConnectedException();
}

ConnectedScoreboardConnector::ConnectedScoreboardConnector(string url)
{
	this->socket = std::make_unique<websocket_client>();
	this->socket->clear_access_channels(websocketpp::log::alevel::all);
	this->socket->set_access_channels(websocketpp::log::alevel::all);

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
				cout << "ERROR:" << ex.what() << endl;
			}
		});

	websocketpp::lib::error_code error_code;
	auto const connection = this->socket->get_connection(url, error_code);

	this->socket->connect(connection);

	//this->socket->start_perpetual();
	
	this->socket->run();
	//thread([this]()
	//	{
	//		this->socket->run();
	//	});
}
