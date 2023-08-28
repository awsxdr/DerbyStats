#pragma once

#include <App.h>

namespace derby_stats::api
{
	typedef function<string()> handler;

	enum class request_type
	{
		get,
		websocket,
	};

	class handler_definition
	{
	private:
		string endpoint_;

	public:
		virtual ~handler_definition() = default;

		[[nodiscard]] virtual request_type request_type() = 0;

		[[nodiscard]] const string& endpoint() const
		{
			return endpoint_;
		}
	};

	class get_handler_definition : public handler_definition
	{
	public:
		[[nodiscard]] api::request_type request_type() override
		{
			return request_type::get;
		}
	};

	class websocket_handler_definition : public handler_definition
	{
	public:
		[[nodiscard]] api::request_type request_type() override
		{
			return request_type::websocket;
		}
	};

	//typedef struct
	//{
	//	request_type request_type;
	//	string endpoint;
	//	handler handler;
	//} handler_definition;

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
				switch (handler.request_type)
				{
				case request_type::get:
					app.get(handler.endpoint, map_handler<SSL>(handler.handler));
					break;

				case request_type::websocket:
					app.ws(handler.endpoint, map_handler<SSL>(handler.handler));
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
