#pragma once

#include <memory>

#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>

namespace derby_stats
{
	using namespace std;

	typedef websocketpp::client<websocketpp::config::asio_client> websocket_client;
	typedef websocketpp::config::asio_client::message_type::ptr message_ptr;

	class ScoreboardConnector
	{
	public:
		virtual ~ScoreboardConnector() = default;

		virtual unique_ptr<ScoreboardConnector> connect(string url) = 0;

		static unique_ptr<ScoreboardConnector> create();
	};

	class DisconnectedScoreboardConnector : public ScoreboardConnector
	{
	public:
		unique_ptr<ScoreboardConnector> connect(string url) override;

		friend class ConnectedScoreboardConnector;
	};

	class ConnectedScoreboardConnector : public ScoreboardConnector
	{
	private:
		unique_ptr<websocket_client> socket;
		thread run_thread;

		ConnectedScoreboardConnector(string url);

	public:
		unique_ptr<ScoreboardConnector> connect(string url) override;

		friend class DisconnectedScoreboardConnector;
	};


}