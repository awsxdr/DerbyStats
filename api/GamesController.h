#pragma once

#include "../ScoreboardConnector.h"
#include "../GameStateStore.h"
#include "ApiController.h"

#include <vector>

using namespace std;

namespace derby_stats::api
{
	class GamesController : public ApiController
	{
	private:
		unique_ptr<GameStateStore> state_store;

	protected:
		vector<handler_definition> get_handlers() override;

	public:
		GamesController(unique_ptr<GameStateStore>& state_store);

		string get_game_state();
	};
}