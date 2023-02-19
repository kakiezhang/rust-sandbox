package main

// NOTE: There should be NO space between the comments and the `import "C"` line.

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import "C"
import (
	"fmt"
	"time"
)

func main() {
	for i := 0; ; i++ {
		C.hello(C.CString(fmt.Sprintf("world: %d", i)))
		C.whisper(C.CString("this is code from the dynamic library"))
		time.Sleep(2 * time.Second)
	}
}
