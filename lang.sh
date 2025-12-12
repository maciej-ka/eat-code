#!/usr/bin/env node

const lang = [
  'C',
  'C++',
  'C#',
  'Dart',
  'Elixir',
  'Erlang',
  'Go',
  'Java',
  'JavaScript',
  'Kotlin',
  'PHP',
  'Python 3',
  'Python',
  'Racket',
  'Ruby',
  'Rust',
  'Scala',
  'Swift',
  'TypeScript',
]

const rand = (array) => {
  const i = Math.floor(Math.random() * array.length);
  return array[i];
};

console.log(rand(lang))
