import java.io.IOException;
import java.lang.String;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.OptionalInt;
import java.util.stream.IntStream;

class Day15 {

	static int hash(String s) {
		return s.chars().reduce(0, (acc, v) -> ((acc + v) * 17) % 256);
	}

	static class Box {
		class BoxEntry {
			String key;
			Integer value;

			BoxEntry(String key, Integer value) {
				this.key = key;
				this.value = value;
			}
		}

		private List<BoxEntry> list;

		Box() {
			list = new ArrayList<>();
		}

		OptionalInt findIndex(String key) {
			return IntStream.range(0, list.size()).filter(i -> list.get(i).key.equals(key)).findFirst();
		}

		void insert(String key, Integer value) {
			findIndex(key).ifPresentOrElse(i -> list.get(i).value = value,
					() -> list.add(new BoxEntry(key, value)));
		}

		void remove(String key) {
			findIndex(key).ifPresent(i -> list.remove(i));
		}

		int summary() {
			return IntStream.range(0, list.size()).map((i) -> (i + 1) * list.get(i).value).sum();
		}

	}
	static int run(String line) {
		var boxes = new Box[256];
		IntStream.range(0, boxes.length).forEach(i -> boxes[i] = new Box());

		Arrays.stream(line.split(",")).map((v) -> v.trim()).forEach(s -> {
			var c = s.charAt(s.length() - 1);
			if (Character.isDigit(c)) {
				var label = s.substring(0, s.length() - 2);
				int digit = c - '0';
				boxes[hash(label)].insert(label, digit);
			} else {
				var label = s.substring(0, s.length() - 1);
				boxes[hash(label)].remove(label);
			}
		});
		return IntStream.range(0, boxes.length).map(i -> (i + 1) * boxes[i].summary()).sum();
	}

	public static void main(String[] args) throws IOException {

		String content = new String(Files.readAllBytes(Paths.get("input.txt")));
		int sum = Arrays.stream(content.split(",")).map((v) -> v.trim()).mapToInt(Day15::hash).sum();

		System.out.println("Part1: %s".formatted(sum));


		System.out.println("Part2: %s".formatted(run(content)));

		for (var i = 0; i < 1000; i++)  {
		    run(content);
		}
		var start = System.nanoTime();
		var count = 20000;
		for (var i = 0; i < count; i++)  {
		    run(content);
		}
		var duration = System.nanoTime() - start;
		System.out.println("Part2 duration avg: %d ns".formatted(duration / count));
	}
}
