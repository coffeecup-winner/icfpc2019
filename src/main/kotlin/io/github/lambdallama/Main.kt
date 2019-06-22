package io.github.lambdallama

import io.github.lambdallama.strategy.*
import io.github.lambdallama.ui.launchGui
import io.github.lambdallama.ui.visualize
import java.io.File

fun nonInteractiveMain(path: String, validate: Boolean) {
    val state = State.parse(File(path).readText())
    println("Map: $path, max points: ${state.maxPoints}")

    val solutions = arrayOf(
        NaiveIterative,
        GreedyUnordered,
        GreedySameMoveFirst,
        GreedySMFTurnover
    ).map {
        val actions = mutableListOf<Action>()
        it.run(state.clone(), actions::plusAssign)
        System.err.println("${it.javaClass.simpleName}: ${actions.size}")
        actions
    }

    val solutionPath = path.substring(0, path.length - 5) + ".sol"
    val solutionFile = File(solutionPath)
    solutionFile.writeText(solutions.minBy { it.size }!!.joinToString(""))

    if (validate) {
        print("Validating... ")
        when (val validationResult = validateWithJsValidator(path, solutionPath)) {
            is JsValidatorResult.Success -> println("OK, ${validationResult.time}")
            is JsValidatorResult.Failure -> println("ERROR: ${validationResult.error}")
        }
    }
}

fun main(args: Array<String>) {
    when (args[0]) {
        "--non-interactive" -> {
            var validate = false
            if (args.count() >= 3 && args[2] == "--validate") {
                validate = true
            }
            return nonInteractiveMain(args[1], validate)
        }
    }

    // ./gradlew run --args=path/to/prob-XXX.desc
    val path = args.firstOrNull() ?: throw IllegalStateException(
        "path/to/prob-XXX.desc")
    val state = State.parse(File(path).readText())

    launchGui()
    GreedySMFTurnover.run(state, visualize(state, true))
}
