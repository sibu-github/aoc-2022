import 'dart:async';
import 'dart:io';
import 'dart:convert';

main() async {
  var lines = await File("inputs.txt")
      .openRead()
      .transform(utf8.decoder)
      .transform(LineSplitter());
  var highest1 = 0;
  var highest2 = 0;
  var highest3 = 0;
  var sum = 0;
  await lines.forEach((line) {
    if (line == "") {
      if (sum > highest1) {
        highest3 = highest2;
        highest2 = highest1;
        highest1 = sum;
      } else if (sum > highest2) {
        highest3 = highest2;
        highest2 = sum;
      } else if (sum > highest3) {
        highest3 = sum;
      }
      sum = 0;
    } else {
      var num = int.parse(line);
      sum += num;
    }
  });

  if (sum > highest1) {
    highest3 = highest2;
    highest2 = highest1;
    highest1 = sum;
  } else if (sum > highest2) {
    highest3 = highest2;
    highest2 = sum;
  } else if (sum > highest3) {
    highest3 = sum;
  }

  print("Highest is: $highest1");
  print("Total of highest 3: ${highest1 + highest2 + highest3}");
}
