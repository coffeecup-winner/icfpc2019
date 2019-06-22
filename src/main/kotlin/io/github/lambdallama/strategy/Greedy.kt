package io.github.lambdallama.strategy

import io.github.lambdallama.*
import java.util.*

private val MOVES = arrayOf(MoveUp, MoveDown, MoveLeft, MoveRight)

private inline fun ByteMatrix.bfs(
    initial: Point,
    shouldStop: (Point) -> Boolean): List<Point> {
    val backtrack = mutableMapOf<Point, Point?>(initial to null)
    val q = ArrayDeque<Point>()
    q.addLast(initial)
    var u: Point? = initial
    while (q.isNotEmpty()) {
        u = q.removeFirst()
        if (shouldStop(u)) {
            break
        }

        for (move in MOVES) {
            val v = move(u)
            if (v in this && !this[v].isObstacle && v !in backtrack) {
                q.addLast(v)
                backtrack[v] = u
            }
        }
    }

    val path = mutableListOf<Point>()
    while (u != null && u in backtrack) {
        path.add(u)
        u = backtrack[u]
    }
    path.reverse()
    return if (path.isNotEmpty() && shouldStop(path.last())) {
        check(path.first() == initial)
        path
    } else {
        emptyList()
    }
}

interface Greedy : Strategy {
    override fun run(state: State, sink: ActionSink) {
        val grid = state.grid
        sink(listOf(TurnClockwise))
        state.apply(TurnClockwise)
        while (true) {
            check(grid[state.robot.position] == Cell.WRAPPED)
            val path = route(state)
            if (path.isEmpty()) {
                break
            }

            follow(state, path, sink)
        }
    }

    fun route(state: State): List<Point> {
        val grid = state.grid
        return grid.bfs(state.robot.position) { grid[it].isWrapable }
    }

    fun follow(state: State, path: List<Point>, sink: ActionSink)
}

object GreedyUnordered: Greedy {
    override fun follow(state: State, path: List<Point>, sink: ActionSink) {
        for (v in path.drop(1)) {
            val move = MOVES.first { it(state.robot.position) == v }
            sink(listOf(move))
            state.apply(move)
            if (state.hasBooster(BoosterType.B)) {
                val attach = Attach(state.robot.attachmentPoint())
                sink(listOf(attach))
                state.apply(attach)
            }
        }
    }
}

fun Rotation.toMove() = when (this) {
    Rotation.CLOCKWISE -> TurnClockwise
    Rotation.COUNTERCLOCKWISE -> TurnCounter
}

object GreedyUnorderedTurnover: Greedy {
    override fun follow(state: State, path: List<Point>, sink: ActionSink) {
        val grid = state.grid

        // Find a starting rotation which maximizes number of wrapped cells per action.
        val rotation = arrayOf(null, Rotation.CLOCKWISE, Rotation.COUNTERCLOCKWISE).maxBy { candidate ->
            var wrapped = 0
            var score = 0
            val clone = state.robot.clone()
            val boosters = HashMap(state.collectedBoosters)
            candidate?.let {
                clone.rotate(it)
                score++
            }
            for (i in 1 until path.size) {
                val v = path[i]
                clone.position = v  // .move is mutating!
                score++
                val boosterType = state.boosters[v]
                if (boosterType != null && boosterType != BoosterType.X) {
                    boosters[boosterType] = boosters[boosterType]!! + 1
                }
                if (boosters[BoosterType.B]!! > 0) {
                    clone.attachTentacle(clone.attachmentPoint())
                    score++
                }
                wrapped += clone.getVisibleParts(grid).count { grid[it].isWrapable }
            }

            wrapped / score.toDouble()
        }

        if (rotation != null) {
            sink(listOf(rotation.toMove()))
            state.apply(rotation.toMove())
        }
        for (i in 1 until path.size) {
            val v = path[i]
            val move = MOVES.first { it(state.robot.position) == v }
            sink(listOf(move))
            state.apply(move)

            if (state.hasBooster(BoosterType.B)) {
                val attach = Attach(state.robot.attachmentPoint())
                sink(listOf(attach))
                state.apply(attach)
            }
        }
    }
}

/** Partition the matrix into components of connected FREE/Booster cells. */
fun ByteMatrix.fbPartition(): List<Set<Point>> {
    val components = mutableListOf<Set<Point>>()
    val seen = mutableSetOf<Point>()
    for (x in 0 until dim.x) {
        for (y in 0 until dim.y) {
            val initial = Point(x, y)
            if (initial !in seen
                && !this[initial].isObstacle && this[initial] != Cell.WRAPPED) {
                val component = mutableSetOf(initial)
                val q = ArrayDeque<Point>()
                q.addLast(initial)
                while (q.isNotEmpty()) {
                    val u = q.removeLast()
                    for (move in MOVES) {
                        val v = move(u)
                        if (v in this && v !in seen
                            && !this[v].isObstacle && this[v] != Cell.WRAPPED) {
                            q.addLast(v)
                            component.add(v)
                            seen.add(v)
                        }
                    }
                }

                components.add(component)
            }
        }
    }
    return components
}

interface GreedyFBPartition : Greedy {
    override fun route(state: State): List<Point> {
        val grid = state.grid
        val components = grid.fbPartition()
        return when (components.size) {
            0 -> emptyList()
            1 -> super.route(state)
            else -> {
                val distances = distanceToAll(grid, state.robot.position)
                val closestComponent = components.minBy { component ->
                    component.map { distances[it]!! }.max()!!
                }!!

                val closestPoint = closestComponent.minBy { distances[it]!! }!!
                val path = grid.bfs(state.robot.position) { it == closestPoint }
                check(path.isNotEmpty())
                return path
            }
        }
    }

    private fun distanceToAll(grid: ByteMatrix, initial: Point): Map<Point, Int> {
        val maxDistance = grid.dim.x * grid.dim.y
        val distances = mutableMapOf<Point, Int>()
        distances[initial] = 0
        val q = ArrayDeque<Point>()
        q.addLast(initial)
        while (q.isNotEmpty()) {
            val u = q.removeFirst()
            for (move in MOVES) {
                val v = move(u)
                if (v in grid && !grid[v].isObstacle) {
                    val alternative = distances[u]!! + 1
                    if (alternative < distances.getOrDefault(v, maxDistance)) {
                        distances[v] = alternative
                        q.addLast(v)
                    }
                }
            }
        }
        return distances
    }
}

object GreedyUnorderedFBPartition : GreedyFBPartition {
    override fun follow(state: State, path: List<Point>, sink: ActionSink) {
        return GreedyUnordered.follow(state, path, sink)
    }
}

object GreedyTurnoverFBPartition : GreedyFBPartition {
    override fun follow(state: State, path: List<Point>, sink: ActionSink) {
        return GreedyUnorderedTurnover.follow(state, path, sink)
    }
}