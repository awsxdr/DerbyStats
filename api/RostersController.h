#pragma once

#include "../ScoreboardConnector.h"
#include "../GameStateStore.h"
#include "ApiController.h"

#include <vector>

namespace derby_stats::api
{
	using namespace std;

	class RostersController : public ApiController
	{
	private:
		shared_ptr<GameStateStore> state_store;

	protected:
		vector<handler_definition> get_handlers() override;

	public:
		RostersController(const shared_ptr<GameStateStore>& state_store);

		string get_rosters() const;
	};
}