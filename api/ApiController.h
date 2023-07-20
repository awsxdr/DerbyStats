#pragma once

#include <App.h>

namespace derby_stats::api
{
	typedef function<string()> handler;

	enum class http_verb
	{
		get,
		post,
		put,
		delete_
	};

	typedef struct
	{
		http_verb verb;
		string endpoint;
		handler handler;
	} handler_definition;

	template<bool SSL>
	function<void(uWS::HttpResponse<SSL>*, uWS::HttpRequest*)> map_handler(const handler handler)
	{
		auto const local_handler = handler;

		return [local_handler](uWS::HttpResponse<SSL>* response, uWS::HttpRequest*)
		{
			auto const result = local_handler();

			response->end(result);
		};
	}

	class ApiController
	{
	private:
		template<bool SSL>
		void init_endpoints_template(uWS::TemplatedApp<SSL>& app)
		{
			auto const handlers = this->get_handlers();

			for (auto& handler : handlers)
			{
				switch (handler.verb)
				{
				case http_verb::get:
					app.get(handler.endpoint, map_handler<SSL>(handler.handler));
					break;

				case http_verb::post:
					app.post(handler.endpoint, map_handler<SSL>(handler.handler));
					break;

				default:
					break;
				}
			}
		}

	protected:
		virtual vector<handler_definition> get_handlers() = 0;

	public:
		virtual ~ApiController() = default;

		virtual void init_endpoints(uWS::TemplatedApp<false>& app)
		{
			init_endpoints_template<false>(app);
		}

		virtual void init_endpoints(uWS::TemplatedApp<true>&& app)
		{
			init_endpoints_template<true>(app);
		}

	};
}
