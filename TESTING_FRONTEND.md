# Testing Guide

This project uses **Vitest** for testing TypeScript/React code.

## Running Tests

```bash
# Run all tests once
npm test

# Run tests in watch mode
npm test -- --watch

# Run tests with UI
npm run test:ui

# Run tests with coverage report
npm run test:coverage
```

## Test Structure

```
src/
├── test/
│   ├── setup.ts          # Global test setup (mocks, cleanup)
│   └── *.test.ts         # Test files
├── lib/
│   ├── api.test.ts       # API wrapper tests
│   └── utils.test.ts     # Utility function tests
└── components/
    └── *.test.tsx        # Component tests
```

## Writing Tests

### Basic Test Example

```typescript
import { describe, it, expect } from "vitest";

describe("MyFunction", () => {
  it("should do something", () => {
    const result = myFunction();
    expect(result).toBe(expected);
  });
});
```

### Testing with Mocked Tauri Commands

The Tauri API is automatically mocked in tests. To customize mock behavior:

```typescript
import { vi } from "vitest";
import { invoke } from "@tauri-apps/api/core";

// Mock a specific command
vi.mocked(invoke).mockResolvedValueOnce({ id: 1, name: "test" });

// Your test that uses the mocked command
const result = await myApiFunction();
expect(result).toEqual({ id: 1, name: "test" });
```

### Component Testing

```typescript
import { render, screen } from "@testing-library/react";
import { MyComponent } from "./MyComponent";

it("renders correctly", () => {
  render(<MyComponent />);
  expect(screen.getByText("Hello")).toBeInTheDocument();
});
```

## Coverage Goals

- **Frontend**: 70%+ test coverage
- Focus on utilities, API wrappers, and complex components
- Test user interactions and state changes

## Best Practices

1. **Test behavior, not implementation**
2. **Use descriptive test names** - "should do X when Y"
3. **One logical assertion per test**
4. **Mock external dependencies** (Tauri commands)
5. **Test error cases** alongside happy paths
