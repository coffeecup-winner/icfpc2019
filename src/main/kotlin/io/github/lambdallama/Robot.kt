package io.github.lambdallama

data class Robot(
    var position: Point,
    val tentacles: MutableList<Point>,
    var orientation: Orientation,
    var fuelLeft: Int
) {
    constructor(position: Point)
        : this(
        position,
        tentacles = mutableListOf(
            Point(1, 0),
            Point(1, 1),
            Point(1, -1)
        ),
        orientation = Orientation.RIGHT,
        fuelLeft = 0
    )


    fun clone() = Robot(position, tentacles.toMutableList(), orientation, fuelLeft)

    fun rotate(rotation: Rotation) {
        orientation = orientation.rotate(rotation)
    }

    fun getVisibleParts(grid: ByteMatrix) = getVisiblePartsAt(grid, position)

    fun getVisiblePartsAt(grid: ByteMatrix, position: Point): Sequence<Point> {
        // TODO: this is a quick, dirty visibility check with false negatives
        // 31024 <- tentacle indices, the algorithm depends on this order
        //   R
        // For each direction, if we encounter a wall on either the tentacles' level or robot level,
        // consider the rest of the tentacles in that direction to be invisible.
        // Tentacles 0-2 are checked in a normal way.
        //
        // Example:
        // 310W4 <- here only 0 and 1 are visible, since
        //  WR
        if (tentacles.none { grid[it].isObstacle }) {
            return sequenceOf(position) + tentacles
        }

        val robotDelta = -tentacles[0].rotate(orientation)
        val hitWall = booleanArrayOf(false, false)
        var idx = 0
        val visible = tentacles.asSequence()
            .map { it.rotate(orientation) + position }
            .filter { p ->
                val wall = p !in grid || grid[p].isObstacle
                val pr = p + robotDelta
                val robotLevelWall = pr !in grid || grid[pr].isObstacle
                if (idx > 0) {
                    hitWall[idx % 2] = hitWall[idx % 2] || wall || robotLevelWall
                }

                val result = !wall && (idx < 3 || !hitWall[idx % 2])
                idx++
                result
            }
        return sequenceOf(position) + visible
    }

    fun attachTentacle(at: Point) {
        tentacles += at.reverseRotate(orientation)
    }

    fun detachLastTentacle() {
        tentacles.removeAt(tentacles.count() - 1)
    }

    fun attachmentPoint(): Point {
        val last = this.tentacles.last()
        return Point(1, if (last.y > 0) -last.y else -last.y + 1).rotate(orientation)
    }
}
