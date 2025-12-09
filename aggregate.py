#!/usr/bin/python

res_path: str = "target_scc/eval_opt.csv"


class EvalResult:
    name: str
    passes: int
    create: int
    switch: int
    time_rewrite: float
    time_noopt: float

    def __init__(self, line: str):
        parts = line.split(",")
        self.name = parts[0].strip()
        self.passes = int(parts[1].strip())
        self.create = int(parts[2].strip())
        self.switch = int(parts[3].strip())
        self.time_rewrite = float(parts[4].strip())
        self.time_noopt = float(parts[5].strip())

    def __str__(self) -> str:
        return "Example: {name}\nNum Passes: {num_pass}\nNum Switch: {num_switch}\nNum Create: {num_create}\nTime No Opt: {t_no}\nTime Opt: {t_opt}".format(
            name=self.name,
            num_pass=self.passes,
            num_switch=self.switch,
            num_create=self.create,
            t_no=self.time_noopt,
            t_opt=self.time_rewrite,
        )

    def __repr__(self) -> str:
        return self.__str__()


class ResultAggregate:
    max_runs: int
    max_runs_names: list[str]

    max_switch: int
    max_switch_names: list[str]

    max_create: int
    max_create_names: list[str]

    max_total: int
    max_total_names: list[str]

    max_time_diff: float
    max_time_diff_names: list[str]

    min_time_diff: float
    min_time_diff_names: list[str]

    avg_switch: int
    avg_create: int
    avg_total: int
    avg_runs: int
    avg_time_tiff: float

    def __init__(self, data: list[EvalResult]):
        self.max_runs = 0
        self.max_switch = 0
        self.max_create = 0
        self.max_total = 0
        self.max_time_diff = 0.0
        self.min_time_diff = float("inf")

        self.max_runs_names = []
        self.max_switch_names = []
        self.max_create_names = []
        self.max_total_names = []
        self.max_time_diff_names = []
        self.min_time_diff_names = []

        total_switch = 0
        total_create = 0
        total_runs = 0
        total_time_diff = 0.0

        for res in data:
            if self.max_runs == res.passes:
                self.max_runs_names.append(res.name)
            elif self.max_runs < res.passes:
                self.max_runs = res.passes
                self.max_runs_names = [res.name]

            if self.max_switch == res.switch:
                self.max_switch_names.append(res.name)
            elif self.max_switch < res.switch:
                self.max_switch = res.switch
                self.max_switch_names = [res.name]

            if self.max_create == res.create:
                self.max_create_names.append(res.name)
            elif self.max_create < res.create:
                self.max_create = res.create
                self.max_create_names = [res.name]

            total = res.switch + res.create

            if self.max_total == total:
                self.max_total_names.append(res.name)
            elif self.max_total < total:
                self.max_total = total
                self.max_total_names = [res.name]

            time_diff = res.time_noopt - res.time_rewrite

            if self.max_time_diff == time_diff:
                self.max_time_diff_names.append(res.name)
            elif self.max_time_diff < time_diff:
                self.max_time_diff = time_diff
                self.max_time_diff_names = [res.name]

            if self.min_time_diff == time_diff:
                self.min_time_diff_names.append(res.name)
            elif self.min_time_diff > time_diff:
                self.min_time_diff = time_diff
                self.min_time_diff_names = [res.name]

            total_switch += res.switch
            total_create += res.create
            total_runs += res.passes
            total_time_diff += time_diff

        total_all = total_switch + total_create
        self.avg_switch = int(round(float(total_switch) / float(len(data))))
        self.avg_create = int(round(float(total_create) / float(len(data))))
        self.avg_total = int(round(float(total_all) / float(len(data))))
        self.avg_runs = int(round(float(total_runs) / float(len(data))))
        self.avg_time_diff = total_time_diff / float(len(data))

    def __str__(self) -> str:
        return """
Max Number of runs: {num_runs} ({max_run_names})
Max Switch Lifts: {num_switch} ({max_switch_names})
Max Create Lifts: {num_create} ({max_create_names})
Max Total Lifts: {num_total} ({max_total_names})
Max Time Diff: {max_time} mus ({max_time_names})
Min Time Diff: {min_time} mus ({min_time_names})
Averge Number of runs: {avg_runs}
Average Switch Lifts: {avg_switch}
Average Create Lifts: {avg_create}
Average Total Lifts: {avg_total}
Average Time Difference: {avg_time} mus
""".format(
            num_runs=self.max_runs,
            max_run_names=", ".join(self.max_runs_names),
            num_switch=self.max_switch,
            max_switch_names=", ".join(self.max_switch_names),
            num_create=self.max_create,
            max_create_names=", ".join(self.max_create_names),
            num_total=self.max_total,
            max_total_names=", ".join(self.max_total_names),
            max_time=round(self.max_time_diff, 2),
            max_time_names=", ".join(self.max_time_diff_names),
            min_time=round(self.min_time_diff, 2),
            min_time_names=", ".join(self.min_time_diff_names),
            avg_runs=self.avg_runs,
            avg_switch=self.avg_switch,
            avg_create=self.avg_create,
            avg_total=self.avg_total,
            avg_time=round(self.avg_time_diff, 2),
        )

    def __repr__(self) -> str:
        return self.__str__()


def load_csv() -> list[EvalResult]:
    csv_file = open(res_path, "r")
    return list(map(lambda line: EvalResult(line), csv_file.readlines()[1:]))


if __name__ == "__main__":
    print(ResultAggregate(load_csv()))
