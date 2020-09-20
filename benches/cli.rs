use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate powerline;

use powerline::{modules::*, theme::SimpleTheme};
use std::time::Duration;

fn benchmark_cli_prompt(c: &mut Criterion) {
	let mut group = c.benchmark_group("CLI Render");
	group.measurement_time(Duration::from_secs(2));
	group.bench_function("init", |b| {
		b.iter(|| {
			let prompt = powerline::Powerline::new();
			black_box(prompt.to_string())
		})
	});

	group.bench_function("all the modules", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(PyVenv::<SimpleTheme>::new()).unwrap();
			prompt.add_module(User::<SimpleTheme>::new()).unwrap();
			prompt.add_module(Host::<SimpleTheme>::new()).unwrap();
			prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false)).unwrap();
			prompt.add_module(Git::<SimpleTheme>::new()).unwrap();
			prompt.add_module(ReadOnly::<SimpleTheme>::new()).unwrap();
			prompt.add_module(Cmd::<SimpleTheme>::new()).unwrap();
			prompt.add_module(ExitCode::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});

	group.bench_function("pyenv", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(PyVenv::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("user", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(User::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("host", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(Host::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("cwd", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false)).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("git", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(Git::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("readonly", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(ReadOnly::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("cmd", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(Cmd::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
	group.bench_function("exitcode", |b| {
		b.iter(|| {
			let mut prompt = powerline::Powerline::new();
			prompt.add_module(ExitCode::<SimpleTheme>::new()).unwrap();
			black_box(prompt.to_string())
		})
	});
}

criterion_group!(benches, benchmark_cli_prompt);
criterion_main!(benches);
