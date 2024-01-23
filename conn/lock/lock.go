package lock

import "time"

type LockState int32

const (
	unlocked LockState = iota
	locked
)

type Lock struct {
	state 		LockState
}

func New() *Lock {
	return &Lock{state: unlocked}
}

func (l *Lock) Acquire() bool {
	for !l.tryAcquire() {
		time.Sleep(1 * time.Nanosecond)
	}

	return true
}

func (l *Lock) tryAcquire() bool {
	if l.state == unlocked {
		l.state = locked
		return true
	}

	return false
}

func (l *Lock) Release() {
	l.state = unlocked
}

func (l *Lock) State() LockState {
	return l.state
}