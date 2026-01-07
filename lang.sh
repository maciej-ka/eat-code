#!/usr/bin/env node

const langs = [
  "Bash",
  "C",
  "C#",
  "C++",
  "Dart",
  "Elixir",
  "Erlang",
  "Go",
  "Haskell",
  "Java",
  "JavaScript",
  "Kotlin",
  "PHP",
  "Python 2",
  "Python 3",
  "Racket",
  "Ruby",
  "Rust",
  "Scala",
  "Swift",
  "TypeScript",
]

const rand = (array) => {
  const i = Math.floor(Math.random() * array.length);
  return array[i];
};

console.log(rand(langs))
