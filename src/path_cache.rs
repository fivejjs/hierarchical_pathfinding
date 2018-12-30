use crate::{NodeID, Point};

mod chunk;
use self::chunk::Chunk;

mod node;
use self::node::Node;

mod cache_config;
pub use self::cache_config::PathCacheConfig;

mod abstract_path;
pub use self::abstract_path::AbstractPath;

mod node_map;
use self::node_map::NodeMap;

/// A struct to store the Hierarchical Pathfinding information.
#[derive(Clone, Debug)]
pub struct PathCache {
	width: usize,
	height: usize,
	chunks: Vec<Vec<Chunk>>,
	nodes: NodeMap,
	config: PathCacheConfig,
}

impl PathCache {
	/// creates a new PathCache
	///
	/// ## Arguments
	/// - ```(width, height)```  the size of the Grid
	/// - ```get_cost```  get the cost for walking over a tile. (Cost < 0 => solid Tile)
	/// - ```config```  optional config for creating the cache
	///
	/// ## Examples
	/// Basic usage:
	/// ```
	/// use hierarchical_pathfinding::PathCache;
	///
	/// // create and initialize Grid
	/// // 0 = empty, 1 = swamp, 2 = wall
	/// let mut grid = [
	///     [0, 2, 0, 0, 0],
	///     [0, 2, 2, 2, 0],
	///     [0, 1, 0, 0, 0],
	///     [0, 1, 0, 2, 0],
	///     [0, 0, 0, 2, 0],
	/// ];
	/// let (width, height) = (grid.len(), grid[0].len());
	///
	/// let cost_map = [
	///     1,  // empty
	///     5,  // swamp
	///     -1, // wall = solid
	/// ];
	///
	/// let mut pathfinding = PathCache::new(
	///     (width, height), // the size of the Grid
	///     |(x, y)| cost_map[grid[x][y]], // get the cost for walking over a tile
	///     Default::default(), // other options for creating the cache
	/// );
	/// ```
	pub fn new(
		(width, height): (usize, usize),
		get_cost: impl Fn(Point) -> isize,
		config: PathCacheConfig,
	) -> PathCache {
		let chunk_hor = {
			let mut w = width / config.chunk_size;
			if w * config.chunk_size < width {
				w += 1;
			}
			w
		};
		let chunk_vert = {
			let mut h = height / config.chunk_size;
			if h * config.chunk_size < height {
				h += 1;
			}
			h
		};

		let mut nodes = NodeMap::new();

		let mut chunks = Vec::with_capacity(chunk_hor);
		for x in 0..chunk_hor {
			let mut row = Vec::with_capacity(chunk_vert);
			let w = if x == chunk_hor - 1 {
				width % config.chunk_size
			} else {
				config.chunk_size
			};

			for y in 0..chunk_vert {
				let h = if y == chunk_vert - 1 {
					height % config.chunk_size
				} else {
					config.chunk_size
				};
				row.push(Chunk::new(
					(x * config.chunk_size, y * config.chunk_size),
					(w, h),
					&get_cost,
					&mut nodes,
				))
			}

			chunks.push(row);
		}

		// TODO: interconnect Nodes

		PathCache {
			width,
			height,
			chunks,
			nodes,
			config,
		}
	}

	/// Calculates the Path from ```start``` to ```goal``` on the Grid.
	///
	/// If no Path could be found, None is returned.
	///
	/// This function takes a mutable reference of self, because ```start``` and ```goal``` need to be inserted into
	/// the Abstract Graph in order for the algorithm to work. They are removed at the end unless
	/// ```keep_insertions``` of PathCacheConfig was set to true (default) when creating the PathCache.
	///
	/// ## Arguments
	/// - ```start```  the Point where the search starts
	/// - ```goal```  the Point to search for. This may be a solid wall.
	/// - ```get_all_neighbors```  a function that takes a Point and returns all neighbors according to the
	/// desired Neighborhood, including solid Tiles, if solid Goals are supposed to be searchable.
	/// See the [neighbors crate](neighbors/index.html) for example functions.
	/// - ```heuristic```  The Heuristic to use while searching. This is usually just the Distance between a Point and the Goal
	/// (see the [neighbors crate](neighbors/index.html)), but it is possible to supply a better Heuristic or just return 0 for
	/// every Point. A better Heuristic greatly improves the speed of the search.
	/// - ```get_cost```  get the cost for walking over a tile. (Cost < 0 => solid Tile)
	///
	/// ## Examples
	/// Basic usage:
	/// ```
	/// # use hierarchical_pathfinding::PathCache;
	/// #
	/// # // create and initialize Grid
	/// # // 0 = empty, 1 = swamp, 2 = wall
	/// # let mut grid = [
	/// #     [0, 2, 0, 0, 0],
	/// #     [0, 2, 2, 2, 2],
	/// #     [0, 1, 0, 0, 0],
	/// #     [0, 1, 0, 2, 0],
	/// #     [0, 0, 0, 2, 0],
	/// # ];
	/// # let (width, height) = (grid.len(), grid[0].len());
	/// #
	/// # let cost_map = [
	/// #     1,  // empty
	/// #     5,  // swamp
	/// #     -1, // wall = solid
	/// # ];
	/// #
	/// # let mut pathfinding = PathCache::new(
	/// #     (width, height), // the size of the Grid
	/// #     |(x, y)| cost_map[grid[x][y]], // get the cost for walking over a tile
	/// #     Default::default(), // other options for creating the cache
	/// # );
	/// #
	/// let start = (0, 0);
	/// let goal = (4, 4);
	///
	/// use hierarchical_pathfinding::neighbors::*;
	///
	/// // find_path returns Some(Path) on success
	/// let path = pathfinding.find_path(
	///     start,
	///     goal,
	///     manhattan_neighbors(width, height), // get_all_neighbors
	///     manhattan_heuristic(goal), // heuristic
	///     |(x, y)| cost_map[grid[x][y]], // cost function
	/// );
	///
	/// assert!(path.is_some());
	/// ```
	pub fn find_path<NeighborIter: Iterator<Item = Point>>(
		&mut self,
		start: Point,
		goal: Point,
		get_all_neighbors: impl Fn(Point) -> NeighborIter,
		heuristic: impl Fn(Point) -> usize,
		get_cost: impl Fn(Point) -> isize,
	) -> Option<AbstractPath> {
		None
	}
}
