package mutex

import "time"

type MutexState int32

const (
	unlocked MutexState = iota
	locked
)

type Mutex struct {
	state 		MutexState	// current state of the mutex
	owner 		int64		// ID of the goroutine that owns the lock
}

func New() *Mutex {
	return &Mutex{
		state: unlocked,
		owner: -999,
	}
}

func (m *Mutex) Acquire(id int64) bool {
	for !m.tryAcquire(id) {
		time.Sleep(1 * time.Nanosecond)
	}

	return true
}

func (m *Mutex) tryAcquire(id int64) bool {
	if m.state != unlocked {
		return false
	}

	m.state = locked
	m.owner = id
	return true
}

func (m *Mutex) Release(id int64) bool {
	if m.owner != id {
		return false
	}

	m.state = unlocked
	m.owner = -999

	return true
}

func (m *Mutex) State() MutexState {
	return m.state
}

func (m *Mutex) Owner() int64 {
	return m.owner
}