package com.theo.hello

object Hello {

    init {
        System.loadLibrary("hello")
    }

    external fun greet(input: String): String

}