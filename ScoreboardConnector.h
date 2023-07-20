#pragma once

#include <memory>

#include <websocketpp/config/asio_no_tls_client.hpp>
#include <websocketpp/client.hpp>

namespace derby_stats
{
	using namespace std;

	typedef websocketpp::client<websocketpp::config::asio_client> websocket_client;
	typedef websocketpp::config::asio_client::message_type::ptr message_ptr;

	typedef function<void(string, string)> state_update_handler;

	class ScoreboardConnector
	{
	protected:
		state_update_handler update_handler;

		virtual bool get_is_open() = 0;

	public:
		virtual ~ScoreboardConnector() = default;

		virtual unique_ptr<ScoreboardConnector> connect(string url) = 0;
		virtual void register_topic(string topic_name) = 0;
		virtual void set_state_update_handler(state_update_handler handler);

		static unique_ptr<ScoreboardConnector> create();
	};

	class DisconnectedScoreboardConnector : public ScoreboardConnector
	{
	protected:
		bool get_is_open() override;

	public:
		unique_ptr<ScoreboardConnector> connect(string url) override;
		void register_topic(string topic_name) override;

		friend class ConnectedScoreboardConnector;
	};

	class ConnectedScoreboardConnector : public ScoreboardConnector
	{
	private:
		unique_ptr<websocket_client> socket;
		websocket_client::connection_ptr connection;
		thread run_thread;
		bool is_open = false;

		ConnectedScoreboardConnector(string url);

		void handle_message(websocketpp::connection_hdl, const message_ptr& message);
		void handle_open(const websocketpp::connection_hdl);

	protected:
		bool get_is_open() override;

	public:
		unique_ptr<ScoreboardConnector> connect(string url) override;
		void register_topic(string topic_name) override;

		friend class DisconnectedScoreboardConnector;
	};


}