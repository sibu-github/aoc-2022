import Foundation

let path = "inputs.txt"
var highest1 = 0
var highest2 = 0
var highest3 = 0
var sum = 0

do {
    let input = try String(contentsOfFile: path, encoding: .utf8)
    let splitted = input.split(separator: "\n", omittingEmptySubsequences: false)
    for s in splitted {
        if(s == ""){
            if(sum > highest1){
                highest3 = highest2
                highest2 = highest1
                highest1 = sum
            } else if (sum > highest2){
                highest3 = highest2
                highest2 = sum
            } else if(sum > highest3){
                highest3 = sum
            }
            sum = 0
            continue
        }
        sum += Int(s) ?? 0
    }
    if(sum > highest1){
        highest3 = highest2
        highest2 = highest1
        highest1 = sum
    } else if (sum > highest2){
        highest3 = highest2
        highest2 = sum
    } else if(sum > highest3){
        highest3 = sum
    }
    let total = highest1 + highest2 + highest3
    print("Highest is: \(highest1)")
    print("Total is: \(total)")
}
catch let error as NSError {
    print("Ooops! Something went wrong: \(error)")
}