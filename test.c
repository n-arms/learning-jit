void consume(float *);

int main() {
  float array[128];

  consume(array);

  array[0] = 1.0;

  consume(array);
}