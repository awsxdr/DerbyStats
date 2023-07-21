#pragma once

#include "ScoreboardConnector.h"

namespace derby_stats
{
	struct skater
	{
		string name = "";
		string number = "";
	};

	struct team
	{
		string name = "";
		int score = 0;
		map<string, skater> skaters;
	};

	struct game_state
	{
		team home_team;
		team away_team;
	};

	class GameStateStore
	{
	private:
		void handle_state_update(string key, string value);
		game_state state;

	public:
		explicit GameStateStore(const unique_ptr<ScoreboardConnector>& scoreboard_connector);

		game_state get_state();
	};
}
