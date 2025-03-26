# 🌟 Project Title: Flashcard and Quiz Website (AI Integration)
## 🍀 Description:

- A website that allows users to create flashcards (questions) for studying.
- Users can create sets of flashcards, add flashcards to sets, and study sets of flashcards.
- Users can also upload a document file to create set of flashcards from the document (AI feature)
- Users can also share set of flashcards public or with restricted users.
- Generate a quiz from a set of questions.
- Generate resource pages from a question and answers

## 👨 Actors: User, Staff

## 🚀 Features: a feature = an actor + a function

- Authentication Management:
  - User -> Sign up by email and password
  - User, Staff -> Log in by email and password
  - User, Staff -> Log out

- Admin Management:
  - Staff -> Get number of users, sets, questions (by types), quizzes
  - Staff -> Get number of quiz tests and average score of all tests
  
- Set Management:
  - User -> Create a set
  - User -> Upload a document file to create set of flashcards (AI feature)
  - User -> Update information of a set
  - User -> Delete a set
  - User -> Share a set with other users or public
  - User -> Get a set with all questions and answers
    > authorised user (owner or shared users)
    > pagination query by page and pageSize (default pageSize = 50)
    > query by: keyword (by question title, answer content)
    > filter by: question types
    > sort by: created_at, title (sort direction: asc, desc), default sort by created_at asc

- Question (Flashcard) Management:
  - User -> Create a question and add it to a set
    > There are 3 types of questions:
    - Text_Fill: fill in the blank
    - Multiple_Choice: choose a correct answer
    - Checkboxes: choose multiple correct answers
    > At least 1 correct answer
  - User -> Update information of a question and all answers of that question
  - User -> Delete a question
  - User -> Get a question with all answers

- Quiz Management:
  - User -> Generate a quiz from a set of questions
    > Input:
    - Number of questions in the quiz
    - Question types in the quiz
  - User -> Submit a quiz
    > Authorised user (owner or shared users)
    > User can leave some questions unanswered
    > If any answer is bad request, don't save quiz result (use transaction)
  - User -> Get all quizes (and shared)
  - User -> Get all quiz results
  - User -> Get a quiz result detail
  - User -> Do a haft quiz, save and continue later
  - User -> Share a quiz with other users or public
