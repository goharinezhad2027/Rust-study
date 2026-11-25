#[derive(Clone, Copy)]
enum Scheduler {
	Fifo,
	Sjf,
}
#[derive(Clone)]
struct Job {
	arrival_time: u32,
	length: u32,
	remained_length: u32,
	state: JobState,
}
#[derive(Clone)]
enum JobState {
	Ready,
	Running,
	Finished,
	‌Blocked,
}

fn times(mut jobs: Vec<Job>, scheduler: Scheduler) -> (Vec<u32>, Vec<u32>) {


	let mut current_cpu_time = 0;
	let mut response_times = Vec::with_capacity(jobs.len());
	let mut turnaround_times = Vec::with_capacity(jobs.len());

	if scheduler == Scheduler::Fifo {
		return timesInFIFO(jobs);
	} else if scheduler == Scheduler::Sjf {
		return timesInSJF(jobs);
	} else {
		panic!("Unknown scheduler");
	}
	
	
}

fn timesInSJF(mut jobs: Vec<Job>, current_cpu_time: u32,response_times: &mut Vec<u32>, turnaround_times: &mut Vec<u32>) -> (Vec<u32>, Vec<u32>){


		while !jobs.is_empty() {


		let job = findNextJob(&jobs, current_cpu_time);
		job.state = JobState::Running;
		response_times.push(current_cpu_time - job.arrival_time);
		current_cpu_time += job.length;
		turnaround_times.push(current_cpu_time - job.arrival_time);
		job.state = JobState::Finished;
		if let Some(index) = jobs.iter().position(|&x|x == job) {
			jobs.remove(index);
		}
	}
	
	(response_times, turnaround_times)
}

fn timesInFIFO(mut jobs: Vec<Job>,current_cpu_time: u32,response_times: &mut Vec<u32>, turnaround_times: &mut Vec<u32>) -> (Vec<u32>, Vec<u32>) {

	current_cpu_time = jobs[0].arrival_time;
	for job in jobs.iter_mut(){
		job.state = JobState::Running;
		response_times.push(current_cpu_time - job.arrival_time);
		current_cpu_time += job.length;
		turnaround_times.push(current_cpu_time - job.arrival_time);
		job.state = JobState::Finished;
	}

}


fn findNextJob(jobs:&Vec<Job>,current_time: u32)->&Job{
	 
	 if(jobs.is_empty()){
		 panic!("No jobs available");
	 }
	 let mut next_job:Option<&Job> = None;
	 if let Some(job) = jobs.iter().filter(|&x|=>x.arrival_time<=current_time).min_by_key(|&x|=> x.length) {
		next_job = Some(job);
	 }
	 If(!next_job.is_none())
	 {
		 return next_job.unwrap();
	 }
	 
	 next_job = jobs.iter().min_by_key(|&x|=> x.arrival_time);
	 current_time = next_job.unwrap().arrival_time;
	 return next_job.unwrap();
		  

}

fn main() {
	// All three jobs arrive at time 0 and have length 200.
	let jobs = vec![Job { arrival_time: 0, length: 800, remained_length: 800, state: JobState::Ready },
	 Job { arrival_time: 1, length: 400, remained_length: 400, state: JobState::Ready }, 
	 Job { arrival_time: 2, length: 200, remained_length: 200, state: JobState::Ready }];

	for (name, scheduler) in [("FIFO", Scheduler::Fifo), ("SJF", Scheduler::Sjf)] {
		let (response, turnaround) = times(jobs.clone(), scheduler);
		println!("{name}: response times {response:?}, turnaround times {turnaround:?}");
	}
}
