#pragma once

#include "../ScoreboardConnector.h"
#include "../state_stores/GameStateStore.h"
#include "ApiController.h"

#include <vector>

namespace derby_stats::api
{
	using namespace std;
	using namespace state_stores;

	class GamesController : public ApiController
	{
	private:
		shared_ptr<GameStateStore> state_store;

	protected:
		vector<handler_definition> get_handlers() override;

	public:
		GamesController(const shared_ptr<GameStateStore>& state_store);

		string get_game_state() const;
	};
}