# User Stories

## Project Description

The existing Status Reporting System of the Shelby website is out-of-date and requires an update to modern standards. Currently, the website can function but is becoming more unmanageable as it starts to show its age. The current user interface is complex and unclear, making it difficult for students to submit weekly reports. Our standalone solution will entirely replace the Status Reporting System component of the website using modern frameworks and features, enabling more effective error messages and an intuitive design for students to complete their status reports. The solution will also give instructors and Admins a more effortless way to create and manage active courses.

*What is user story?*

> *A user story is an informal, general explanation of a software  feature written from the perspective of the end user. Its purpose is to  articulate how a software feature will provide value to the customer.*

## Table of contents (Quick Links)

- [Project Description](#project-description)
- [TLDR](#tldr-min-required-user-stories)
- [Glossary](#glossary)
- [Student Stories](#student-1-need)
- [Instructor Stories](#instructor-1-need)
- [Admin Stories](#Admin-1-need)

## TLDR: Min Required User Stories

- Student
  - Submitting Individual Reports
    - Let user know as they fill it out when not done correct
  - Submitting Team Reports
    - Let user know as they fill it out when not done correct
  - Review old reports
  - Get public portions of team reports
    - Note can reuse Instructor get team report
- Instructor
  - update students
    - using a csv file
  - manage teams
  - get team reports
    - include individual reports
    - include private portions
- Admin
  - add classroom
    - Include adding instructor
  - get server errors
    - by email
  - setup/update server
    - Using the Linux terminal

## Glossary

- Agents/User:
  - Students: the students completing their weekly status reports
  - Instructors: the professors that analyze the status reports to give feedback
  - Administrator (admin): the manager of the website, superusers
- Terms/Projects: A group of students in a class that there reports interact 
- Parsing: deserialization of input data, pulling in new data
- CSV file: a data file that has a table, typically containing a class roster for this example

## **Student #1,** **NEED**

![diagram](Diagrams/student1.jpg)

| **Use Case Title:**  | Individual report                                            |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Student                                                      |
| **Purpose:**         | Submitting individual report as student                      |
| **Description:**     | Used for submitting a required weekly individual report as student |
| **Pre-conditions:**  | Student should exist, student should be enrolled in a class, and student should be in a team, student should be assigned a report assignment |
| **Task Sequence:**   | 1. Student opens Shelby in browser<br />2.R1: Student logs in<br />3.R2: Student gets sent to student page<br />4.R3: get page request<br />5.R4: generate page<br />6.R5: query database <br />7. Student enters the Individual report<br />8.R6: receive the generated page for individual report <br />9. R7: check validation. <br />10. R8: submit the Individual report<br/>11.R7: check validation <br />12. R9: submit request to MySQL<br/>13.R8: Individual report submitted<br />14. confirmation report was submitted correctly |
| **Alternate Flow:**  | 1. Student does not complete the form entirely, and returns later to finish<br />2. Student does not click on submit<br />3. Student clicks on cancel |
| **Post-conditions:** | 1. The form needs to be verified for completion<br />2. Client-side error should be displayed on any incomplete fields<br />3. The report will be submitted and saved |

## **Student #2,**  **NEED**

![diagram](Diagrams/student2.jpg)

| **Use Case Title:**  | Team report                                                  |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Student                                                      |
| **Purpose:**         | Submitting team report as student                            |
| **Description:**     | Submitting a required team report as single student for entire team |
| **Pre-conditions:**  | Student should exist, student should be enrolled in a class, student should be in a team, student should be assigned a report assignment |
| **Task Sequence:**   | 1. Student opens Shelby in browser<br />2.R1: Student logs in<br />3.R2: Student gets sent to student page<br />4.R3: get page request<br />5.R4: generate page<br />6.R5: query database <br />7. Student enters the Team report<br />8.R6: receive the generated page for individual report <br />9. R7: check validation. <br />10. R10: Team submit the report<br/>11.R7: check validation <br />12. R9: submit request to MySQL<br/>13.R10: Team report submitted<br />14. confirmation report was submitted correctly |
| **Alternate Flow:**  | 1. Student does not complete the form entirely, and returns later to finish<br />2. Student does not click on submit<br />3. Student clicks on cancel |
| **Post-conditions:** | 1. The form needs to be verified for completion<br />2. Client-side error needs to be displayed on any incomplete fields<br />3. The report needs to be submitted and saved 4. The report needs to be updated and be viewable for all team members |

## **Student #3,**  **NEED**

![diagram](Diagrams/student3.jpg)

| **Use Case Title:**  | Receiving emails about reports                               |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Student                                                      |
| **Purpose:**         | Creating a record of submissions                             |
| **Description:**     | Student receives an email for all individual and team report submissions |
| **Pre-conditions:**  | Student should exist, student should be enrolled in class, student should be in a team, student's email should be attached to SRS account |
| **Task Sequence:**   | 1. Student submits report<br />2. R5: database query<br />3. R11: deadline reached<br />4. R12: gen email<br />5. R13 send email<br />6. you got mail! |
| **Alternate Flow:**  | N/A                                                          |
| **Post-conditions:** | N/A                                                          |

## **Student #4,**  **NEED**

![diagram](Diagrams/student4.jpg)

| **Use Case Title:**  | Viewing prior submissions                                    |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Student                                                      |
| **Purpose:**         | Making it possible to view prior submissions                 |
| **Description:**     | Student can view prior submissions via weekly report schedule |
| **Pre-conditions:**  | Student exists, student is enrolled in class, student is in a team, and student has completed a report |
| **Task Sequence:**   | 1. Student opens Shelby in browser<br />2.R1: Student logs in<br />3.R2: Student gets sent to student page<br />4.R3: get page request<br />5.R4: generate page<br />6.R5: query database <br />7. Student enters the old Team report<br /> |
| **Alternate Flow:**  | N/A                                                          |
| **Post-conditions:** | N/A                                                          |



---

## **Instructor #1,** **NEED** 

![diagram](Diagrams/instuctor1.jpg)

| **Use Case Title:**  | updating rooster                                             |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Instructor                                                   |
| **Purpose:**         | To update the class rooster                                  |
| **Description:**     | updating the list of students in the class                   |
| **Pre-conditions:**  | Instructor should exist, admin should give instructor required permissions |
| **Task Sequence:**   | R1: Instructor logs in <br />R14: Instructor gets sent to Admin page<br />R15: get page request<br />R16: generate page<br />R5: query database <br />R17. upload csv of the student rooster and converts it<br />R18. submit the update student rooster to the server<br />R19. submit the student rooster to the Database |
| **Alternate Flow:**  | 1. Instructor hits cancel <br />2. Instructor does not correctly complete form and has to retry |
| **Post-conditions:** | 1. The form needs to be parsed and class needs to be created<br />2. The class needs to be displayed to the instructor in organized fashion |



## **Instructor #2,**  **NEED**

![diagram](Diagrams/instuctor2.jpg)

| **Use Case Title:**  | adding teams                                                 |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Instructor                                                   |
| **Purpose:**         | To add student to teams                                      |
| **Description:**     | Using a CSV file to add Teams and students to teams          |
| **Pre-conditions:**  | Instructor should exist, admin should give instructor required permissions, instructor should have CSV file of team list |
| **Task Sequence:**   | R1: Instructor logs in <br />R14: Instructor gets sent to Admin page<br />R15: get page request<br />R16: generate page<br />R5: query database <br />R17. upload csv of teams and converts it<br />R20. submit the update teams to the server<br />R21. submit the teams to the Database |
| **Alternate Flow:**  | Instructor hits cancel button                                |
| **Post-conditions:** | 1. CSV file needs to be parsed<br />2. Class creation form should decide relevant details about a class such as class name, class duration, etc |



## **Instructor #3,**  **NEED**

![diagram](Diagrams/instuctor3.jpg)

| **Use Case Title:**  | email                                                        |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | Instructor                                                   |
| **Purpose:**         | get email of report                                          |
| **Description:**     | getting email of the report                                  |
| **Pre-conditions:**  | Instructor should have an email setup to receive emails      |
| **Task Sequence:**   | R11: the deadline set on the server passes<br/>R5: the server queries the database for the individual and team reports<br/>R12: the server generates the emails with that individual and team reports<br/>R13: sends email to an email servers |
| **Alternate Flow:**  | Instructor hits cancel button                                |
| **Post-conditions:** | Instructor gets reports                                      |



---

## **ADMIN #1, NEED**

![diagram](Diagrams/admin1.jpg)

| **Use Case Title:**  | deploying product                                            |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | ADMIN                                                        |
| **Purpose:**         | to setup the website                                         |
| **Description:**     | setup for the website                                        |
| **Pre-conditions:**  | needs an environment to setup the server                     |
| **Task Sequence:**   | R1. Admin needs to login.<br />R24. makes the release build<br />R25. setup <br />R26. docs for software use |
| **Alternate Flow:**  | n/a                                                          |
| **Post-conditions:** | 1: web server is setup                                       |



## **Admin #2, NEED**

![diagram](Diagrams/admin2.jpg)

| **Use Case Title:**  | updating courses                                             |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | ADMIN                                                        |
| **Purpose:**         | Be able to manage courses                                    |
| **Description:**     | The ADMIN will setup the courses                             |
| **Pre-conditions:**  | The ADMIN need a csv file of courses                         |
| **Task Sequence:**   | R1: authorized user login.<br/>R14: view authorized users page.<br/>R15: get the users page from the server.<br/>R16: Generates the users page at the server.<br/> R5: queries the database <br/>R17: uploads and converts a CSV file for the course<br/>R22: submits the courses updates to the server<br/>R23: server submits changes to the database. |
| **Alternate Flow:**  | N/A                                                          |
| **Post-conditions:** | 1: the course is now ready to be filled by the teacher with students and teams. |



## **ADMIN #3, NEED**

![diagram](Diagrams/admin3.jpg)

| **Use Case Title:**  | Emails                                                       |
| -------------------- | ------------------------------------------------------------ |
| **Actors:**          | ADMIN                                                        |
| **Purpose:**         | To receive error log emails                                  |
| **Description:**     | To be able to change the reports and even answers to the report |
| **Pre-conditions:**  | The ADMIN should be able to edit reports                     |
| **Task Sequence:**   | R27: An error is registered/detected on the server. <br />R28: A log of the error is created and stored within the log <br />R29: an e-mail is generated by the server detailing the error <br />R30: an error query is sent to the database. |
| **Alternate Flow:**  | 1: ADMIN edits the report but doesn't click save<br />2: If they leave, ask ADMIN if they want to save. |
| **Post-conditions:** | ADMIN changed the report                                     |

