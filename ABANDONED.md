This branch was abandoned for the following reasons:

- **Complexity**: The lazy initialization version introduced additional complexity for subsequent operations such as indexing and matrix calculations.
- **Performance**: The constructor method is called infrequently, and the performance gains from uninitialized memory are outweighed by the additional branch jumping overhead incurred during matrix element traversal.
