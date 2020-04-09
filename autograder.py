#! /usr/bin/env python
import argparse
from pathlib import Path
from typing import List
from collections import namedtuple
import json

TestReport = namedtuple("TestReport", "score max_score name label output ")

Report = namedtuple("TestReport", "score tests")


def main(build_output_paths: List[Path], scores_path: Path, output: Path):
    def test_reports():
        with scores_path.open() as scores_file:
            for i, (score, output_path) in enumerate(
                zip(scores_file.readlines(), build_output_paths)
            ):
                with output_path.open() as output_file:
                    yield TestReport(
                        score=1 - int(score),
                        max_score=1,
                        name=f"Problem {i}",
                        label="",
                        output=output_file.read(),
                    )

    with output.open("w") as f:
        tests = list(test_reports())
        score = sum(t.score for t in tests)
        json.dump(Report(score=score, tests=[t._asdict() for t in tests])._asdict(), f)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-output-paths", type=Path, nargs="*")
    parser.add_argument("--scores-path", type=Path)
    parser.add_argument("--output", type=Path)
    main(**vars(parser.parse_args()))
