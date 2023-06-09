Database documentation
by Jonathan

The database version 0.5 is built on the idea of the user well be verified through Ouath which is in the table of users and called ouath_id. Then the table email is used to navigate the rest of the tables. The reason of using the email as the primary what of traversing the database is that Ouath id will not be set until the user has login in for the first time.

[Development Environment](Database_User_Guide.md)

![DB SCHEMA](https://i.imgur.com/UANnVbm.png)

Tables 
- users purpose: is to store the users, link them to their reports, teams, login to the correct user, and receive emails.
	- ouath_id: is varchar (255): This hold the token return when a person logins in the the website.(in the database but feature is not for this is not implemented)
	- is_theacher: is a boolean: that allows the user to gain access to teacher privileges. Also help in the sending of emails to the correct teacher. (in the database but feature is not for this is not implemented)
	- is_student: is a boolean: that allows the user to gain access to student privileges. Also help the server in querying the reports for the students. (in the database but feature is not for this is not implemented)
	- is_admin: is a boolean: that allows the user to gain access to is_admin privileges. (in the database but feature is not for this is not implemented)
	- teams: is a varchar(50): that allows a user to be added to a team. 
	- class: is a varchar(50): places the user in a class. This helps with the server being able to email the correct teacher..
	- email: varchar(255): allows the user to be emailed and to navigate the database 


- spirit_num_dates: has two purpose to keeping all the reports for a week link to each other, and it allows the server to know when to unlock and lock the reports
	- spirit_num: is an int: that allows the induvial report, team report, and requirements  to be linked together and on the same time.
	- spirit_dates: is an int: allows the spirit_num to be linked to a date.
	
- individual_reports: allows the user to make weekly reports for themselves.
	  the following variables are part of the individual reports as the data fields of the report.
	- monday_time, tuesday_time, wedensday_time, thursday_time, friday_time, saturday_time, sunday_time, discrepancy, request
	
	- email: varchar(255): is present to allow the user to reach this table.
	- spirit_num: is an int: does two things it allows for a unique entry for each report and allows the server to know which report to save to.

- team_activities:  was to allow users to comment on teammates  however this hase been determined to not be adequate.
	- spirit_num: is an int: allows the the correct team report to 
	- activity_index: is an auto-indexing int: to provide a unique number per activaties
	- answer: is a varchar(255): the respond to a teammates stated activates 
	- teams: is an varchar(50): that links the team to the correct activity
	-  email: varchar(255): is present to allow the user to reach this table.

- team_reports: allows the user to make weekly reports for team
	   the following variables are part of the team reports as the data fields of the report.
	-  understand_easiest, understand_hardest, approach_easiest, approach_hardest, solve_easiest, solve_hardest, evaluate_easiest, evaluate_hardest, completion, contact, comments
	
	- teams: varchar(50): this ensure the right team/members connects to this report.
	- spirit_num: int: to link to the correct spirit this week.

- requirements: this table hold the requirements for the teams project however this hase been determined to not be adequate.
	- description: varchar(255):  This describes the requirement for the users projects 
	- teams : varchar(50) :
	- indexs : int : to allows different requirement for each team.


