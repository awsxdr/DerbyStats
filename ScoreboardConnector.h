#pragma once

#include <ClientApp.h>
#include <memory>

namespace derby_stats
{
	using namespace std;

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
		ConnectedScoreboardConnector(uWS::ClientApp client);

	public:
		unique_ptr<ScoreboardConnector> connect(string url) override;

		friend class DisconnectedScoreboardConnector;
	};


}