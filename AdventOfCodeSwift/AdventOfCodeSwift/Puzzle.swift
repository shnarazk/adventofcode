//
//  Solver.swift
//  AdventOfCodeSwift
//
//  Created by 楢崎修二 on 2022/06/17.
//

import Foundation
import SwiftUI
import WebKit

struct PuzzleDescripter: Hashable, Identifiable {
    static func == (lhs: PuzzleDescripter, rhs: PuzzleDescripter) -> Bool {
        lhs.year == rhs.year && lhs.day == rhs.day
    }
    func hash(into hasher: inout Hasher) {
        hasher.combine(self.url)
        hasher.combine(self.title)
    }
    var year: Int
    var day: Int
    var title: String = ""
    var url: String {
        get {
            return "https://adventofcode.com/\(self.year)/day/\(self.day)"
        }}
    var tags: [String]?
    var status: Int = 0
    var solver: (any Solver)?
    var id: UUID = UUID()
    init(year: Int, day: Int, title: String, solver: Solver?) {
        self.year = year
        self.day = day
        self.title = title
        self.solver = solver
    }
}

#if !os(macOS)
struct PuzzlePageView: UIViewRepresentable {
    var url: String
}

struct PuzzleView: View {
    @State var puzzle: PuzzleDescripter
    @State var part1: String?
    @State var part2: String?
    var body: some View {
        VStack {
            Text("Year: \(puzzle.year), Day: \(puzzle.title)")
            Text("URL: \(puzzle.url)")
                .padding(.vertical)
            Text("Description")
            Spacer()
            Section {
                Button("Run part 1") {
                    guard let solver = puzzle.solver else { return }
                    solver.reset()
                    part1 = solver.part1() ?? "Not yet implemented"
                }
                Text(part1 ?? "")
            }
            Section {
                Button("Run part 2") {
                    guard let solver = puzzle.solver else { return }
                    solver.reset()
                    part1 = solver.part2() ?? "Not yet implemented"
                }
                Text(part2 ?? "")
            }
        }
    }
}

struct PuzzleView_Previews: PreviewProvider {
    static var previews: some View {
        PuzzleView(puzzle: PuzzleDescripter(year: 2022, day: 1, title: "test", solver: nil))
    }
}

#endif

