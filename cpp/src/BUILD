package(default_visibility = ["//visibility:public"])

cc_library(
        name = "larry",
        srcs = ["larry/6809.C"],
        hdrs = ["larry/EMU.H", "larry/larry.h"],
        includes = ["./larry"],
        )


cc_binary(
        name = "core",
        srcs = ["core/main.cpp", 
                "core/mem.cpp", "core/c6809base.cpp", "core/c6809Larry.cpp",
                "core/mem.h", "core/c6809base.h","core/c6809Larry.h",
                "core/sha1.hpp",
                ],
        deps = [":larry"]
        )

