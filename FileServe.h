#pragma once 

#include <App.h>
#include <filesystem>
#include <fstream>

namespace derby_stats::file_serve
{
	using namespace std;
	using namespace std::filesystem;

	string_view get_content_type_for_file_extension(path file_extension);
	bool is_parent_path(const path& parent_path, const path& candidate_path);
	tuple<bool, path> get_request_file_path(const path& serve_path, const string_view& url);
	stringstream get_file_contents(const path& file_path);

	template <bool Ssl>
	std::function<void(uWS::HttpResponse<Ssl>*, uWS::HttpRequest*)> handle(const path& serve_path)
	{
		return [serve_path](uWS::HttpResponse<Ssl>* response, uWS::HttpRequest* request)
		{
			auto [path_is_valid, request_path] =
				get_request_file_path(serve_path, request->getUrl());

			if (!path_is_valid)
			{
				response->writeStatus("404");
				response->endWithoutBody(nullopt, true);
				return;
			}

			if (!request_path.has_filename() || !filesystem::exists(request_path))
			{
				request_path = "ui/index.html";
			}

			response->writeHeader("Content-Type", get_content_type_for_file_extension(request_path.extension()));

			auto const file_contents = get_file_contents(request_path);

			response->end(file_contents.str());
		};
	}

	inline stringstream get_file_contents(const path& file_path)
	{
		const ifstream file_stream(file_path);
		stringstream buffer;

		buffer << file_stream.rdbuf();

		return buffer;
	}

	inline tuple<bool, path> get_request_file_path(const path& serve_path, const string_view& url)
	{
		string_view local_url;
		copy(url, local_url);

		while (url.starts_with('/'))
			local_url.remove_prefix(1);

		auto const url_string = string("ui/").append(local_url);

		auto request_path = filesystem::absolute(url_string);

		if (request_path == "")
		{
			copy(serve_path, request_path);
		}

		if (!is_parent_path(serve_path, request_path))
		{
			return { false, "" };
		}

		return { true, request_path };
	}

	inline bool is_parent_path(const path& parent_path, const path& candidate_path)
	{
		auto const empty_path = path();
		auto current_path = candidate_path;

		while (current_path != empty_path)
		{
			if (current_path == parent_path)
				return true;

			auto const last_path = current_path;
			current_path = current_path.parent_path();
			if (last_path == current_path)
				break;
		}

		return false;
	}

	inline string_view get_content_type_for_file_extension(const path file_extension)
	{
		if (file_extension == ".html" || file_extension == ".htm")
			return "text/html";

		if (file_extension == ".css")
			return "text/css";

		if (file_extension == ".js")
			return "text/javascript";

		return "text/plain";
	}
}
