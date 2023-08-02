import Foundation


@objc(PyxelRustModule)
class PyxelRustModule: RCTEventEmitter {
  
  // Returns an array of your named events
  override func supportedEvents() -> [String]! {
    return ["message"]
  }
  
  // Takes an errorCallback as a parameter so that you know when things go wrong.  
  // This will make more sense once we get to the Javascript
  @objc func doSomethingThatHasMultipleResults(
    _ errorCallback: @escaping RCTResponseSenderBlock) {
    
    let failure = { (error: Error) in errorCallback([error]) }
    
    let names = ["Anna", "Alex", "Brian", "Jack"]
    for name in names {
      
      if name != "Jack" {
        // You send an event with your result instead of calling a callback
        self.sendEvent(withName: "MyEvent", body: name)
      }
      else {
        
        // This is only here to show how to use the errorCallback.
        // Pretend that Jack is no good, so something went wrong
        
        struct MyError: Error {
          enum ErrorKind {
            case JackSucks
          }
          let line: Int
          let kind: ErrorKind
        }
        
        failure(MyError(line: 44, kind: .JackSucks))
      }
    }
  }
}
