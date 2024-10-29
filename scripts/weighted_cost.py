import sys

def parse_cost_line(line):
  """Parses a line from the cost file and returns a tuple (instruction, cost)."""
  parts = line.split(":")
  instruction = parts[0].strip()
  cost = int(parts[1].strip().split(",")[0])
  return instruction, cost

def compute_weighted_cost(cost_file, instruction_costs={"i32.const": 0, "i64.const": 0}):
  """
  Computes the weighted cost of instructions in the cost file.

  Args:
      cost_file: Path to the cost file.
      instruction_costs: A dictionary where keys are instruction names and values are their cost (default 1).

  Returns:
      A dictionary where keys are instructions and values are their weighted costs.
  """
  total_cost = 0
  with open(cost_file, "r") as f:
    # Skip the first line (Total line)
    next(f)

    for line in f:
      instruction, cost = parse_cost_line(line)
      weighted_cost = cost * instruction_costs.get(instruction, 1)
      total_cost += weighted_cost

  return total_cost

if __name__ == "__main__":
  cost_file = sys.argv[1]
  instruction_costs = {
      "local.get": 0,
      "local.set": 0,
      "local.tee": 0,
      "i32.const": 0,
      "i64.const": 0,
      "i32.load": 0,
      "i64.load": 0,
      "i32.store": 0,
      "i64.store": 0,
  }

  weighted_cost = compute_weighted_cost(cost_file, instruction_costs)
  print(f"Weighted cost: {weighted_cost}")
