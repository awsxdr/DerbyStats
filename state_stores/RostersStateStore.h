#pragma once

#include "StateStore.hpp"
#include "../ScoreboardConnector.h"

namespace derby_stats::state_stores
{
	struct roster_skater
	{
		string name;
		string number;
	};

	struct roster_team
	{
		string name;
		map<string, roster_skater> skaters;
	};

	struct roster_state
	{
		roster_team home_team;
		roster_team away_team;
	};

	class RostersStateStore : public StateStore<roster_state>
	{
	public:
		explicit RostersStateStore(const shared_ptr<ScoreboardConnector>& scoreboard_connector);
	};
}