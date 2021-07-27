package com.test.benchmark

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class BenchmarkApplication

fun main(args: Array<String>) {
	runApplication<BenchmarkApplication>(*args)
}
