# Java Solution with Unit Tests

This project contains a Java solution with comprehensive unit testing using JUnit 5.

## Project Structure

```
├── src/
│   ├── main/java/
│   │   └── Solution.java          # Main solution class
│   └── test/java/
│       └── SolutionTest.java      # Unit tests
├── pom.xml                        # Maven configuration
├── run-tests.sh                   # Script to run tests
└── README.md                      # This file
```

## Running Tests

### Option 1: Using the script
```bash
./run-tests.sh
```

### Option 2: Using Maven directly
```bash
mvn test
```

### Option 3: Run specific test
```bash
mvn test -Dtest=SolutionTest#testPartitionArrayBasic
```

## Test Coverage

The test suite includes:
- Basic functionality test
- Empty array test
- Single element test
- Large array test
- Negative numbers test

## Requirements

- Java 11 or higher
- Maven 3.6 or higher

## Setup

If you don't have Maven installed, you can install it via:
- **macOS**: `brew install maven`
- **Ubuntu/Debian**: `sudo apt install maven`
- **Windows**: Download from https://maven.apache.org/download.cgi 