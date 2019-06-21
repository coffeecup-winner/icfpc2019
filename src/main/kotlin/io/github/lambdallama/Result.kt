package io.github.lambdallama

interface Action

interface Move: Action {
    val dx: Int
    val dy: Int
    val flipped: Move
}

object MoveUp: Move {
    override val dx = 0
    override val dy = +1
    override val flipped = MoveDown

    override fun toString() = "W"
}

object MoveDown: Move {
    override val dx = 0
    override val dy = -1
    override val flipped = MoveUp

    override fun toString() = "S"
}

object MoveLeft: Move {
    override val dx = -1
    override val dy = 0
    override val flipped = MoveRight

    override fun toString() = "A"
}

object MoveRight:Move {
    override val dx = +1
    override val dy = 0
    override val flipped = MoveLeft

    override fun toString() = "D"
}

object TurnClockwise: Action {
    override fun toString() = "E"
}

object TurnCounter: Action {
    override fun toString() = "Q"
}

data class Attach(val location: Point) : Action {
    override fun toString() = "B$location"
}

object Accelerate : Action {
    override fun toString() = "F"
}

object Drill : Action {
    override fun toString() = "L"
}

class Teleport(private val location: Point) : Action {
    override fun toString() = "R$location"
}

object NoOp : Action {
    override fun toString() = "Z"
}

typealias ActionSink = (Action) -> Unit