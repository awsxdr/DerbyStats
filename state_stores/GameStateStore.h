#pragma once

#include "../ScoreboardConnector.h"
#include "StateStore.hpp"

namespace derby_stats::state_stores
{
	struct game_team
	{
		string name;
		int score = 0;
	};

	struct game_state
	{
		game_team home_team;
		game_team away_team;
	};

	class GameStateStore : public StateStore<game_state>
	{
	public:
		explicit GameStateStore(const shared_ptr<ScoreboardConnector>& scoreboard_connector);
	};
}
