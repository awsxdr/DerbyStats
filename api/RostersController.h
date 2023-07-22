#pragma once

#include "../ScoreboardConnector.h"
#include "../state_stores/RostersStateStore.h"
#include "ApiController.h"

#include <vector>

namespace derby_stats::api
{
	using namespace std;
	using namespace state_stores;

	class RostersController : public ApiController
	{
	private:
		shared_ptr<RostersStateStore> state_store;

	protected:
		vector<handler_definition> get_handlers() override;

	public:
		RostersController(const shared_ptr<RostersStateStore>& state_store);

		string get_rosters() const;
	};
}