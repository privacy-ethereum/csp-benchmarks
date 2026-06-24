import XCTest

final class MoproAppUITests: XCTestCase {
  override func setUpWithError() throws {
    continueAfterFailure = false
  }

  func testLeanvmPrivateTxProof() throws {
    let app = XCUIApplication()
    app.launch()

    let button = app.buttons["proveLeanvmPrivateTx"]
    XCTAssertTrue(button.waitForExistence(timeout: 10))
    button.tap()

    let log = app.staticTexts["proof_log"]
    XCTAssertTrue(log.waitForExistence(timeout: 10))

    let completed = NSPredicate(format: "label CONTAINS %@", "prove:")
    expectation(for: completed, evaluatedWith: log)
    waitForExpectations(timeout: 600)
  }
}
