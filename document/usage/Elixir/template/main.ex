defmodule Main do
  def main do
    result = solve()
    IO.puts(result)
  end

  def solve() do
    ""
  end
end

defmodule InputHelper do
  def read_integer() do
    IO.gets("") |> String.trim() |> String.to_integer()
  end

  def read_integer_list() do
    IO.gets("")
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end
end

Main.main()
