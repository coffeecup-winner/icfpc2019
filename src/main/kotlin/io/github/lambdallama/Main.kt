package io.github.lambdallama

import io.github.lambdallama.strategy.*
import io.github.lambdallama.ui.launchGui
import io.github.lambdallama.ui.visualize
import java.io.File

fun nonInteractiveMain(
        path: String,
        validate: Boolean,
        showBonusCount: Boolean,
        infoOnly: Boolean
) {
    val state = State.parse(File(path).readText())
    println("Map: $path, dim: ${state.grid.dim}, max points: ${state.maxPoints}")
    if (showBonusCount) {
        println("Bonus count: ${bonusCount(state)}")
    }
    if (infoOnly) {
        return
    }

    val solutions = arrayOf(
        NaiveIterative,
        GreedyUnordered,
        GreedyUnorderedTurnover,
        GreedyUnorderedFBPartition,
        GreedyTurnoverFBPartition
    ).map { strategy ->
        val actions = mutableListOf<Action>()
        strategy.run(state.clone()) { actions += it.first()!! }
        System.err.println("${strategy.javaClass.simpleName}: ${actions.size}")
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
            var showBonusCount = false
            var infoOnly = false
            for (arg in args.drop(2)) {
                when (arg) {
                    "--validate" -> validate = true
                    "--show_bonus_count" ->  showBonusCount = true
                    "--info_only" -> infoOnly = true
                }
            }
            return nonInteractiveMain(
                    path=args[1],
                    validate=validate,
                    showBonusCount = showBonusCount,
                    infoOnly = infoOnly
            )
        }
    }

    // ./gradlew run --args=path/to/prob-XXX.desc
    val path = args.firstOrNull() ?: throw IllegalStateException(
        "path/to/prob-XXX.desc")
    val state = State.parse(File(path).readText())

    launchGui()
    GreedyTurnoverFBPartition.run(state, visualize(state, false))
}
