def exception_factory(exception, message):
    return exception(message)

raise exception_factory(ValueError, "invalid value")
