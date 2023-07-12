#pragma once

#include <stdexcept>

class AlreadyConnectedException : public std::logic_error
{
public:
	AlreadyConnectedException()
	: logic_error("Connection to scoreboard is already established")
	{}
};