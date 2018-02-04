package(default_visibility = ["//visibility:public"])

cc_library(
        name = "larry",
        srcs = ["larry/6809.C"],
        hdrs = ["larry/EMU.H", "larry/larry.h"],
        includes = ["./larry"],
        )


cc_binary(
        name = "core",
        srcs = ["core/main.cpp"],
        deps = [":larry"]
        )

