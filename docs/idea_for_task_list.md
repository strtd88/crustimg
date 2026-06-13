## [Feature: Command line]
### Test Cases
- [] [Test] Empty email → 400  
  - Failing: `assert response.status == 400`  
- [ ] [Test] Invalid password → 401  
- [ ] [Test] Valid creds → 200 + JWT  
