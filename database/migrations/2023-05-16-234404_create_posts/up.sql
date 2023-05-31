create table users
(
    email      varchar(255) not null primary key,
    ouath_id   varchar(255) null,
    is_teacher bool   null,
    is_student bool   null,
    is_admin   bool   null,
    teams      varchar(50) null,
    class      varchar(50)  null,
    first_name varchar(30)  null,
    last_name  varchar(30)  null,
    index idx_teams(teams)
);

create table sprint_num_dates
(
    sprint_num  int  not null primary key,
    sprint_date date not null
);

create table team_activities
(
    teams          varchar(50)  not null,
    email       varchar(255)    not null,
    sprint_num         int      null,
    activity_index int          null,
    answers        varchar(255) null,
    primary key (teams,email),
    constraint team_activities_ibfk_1
        foreign key (teams) references users(teams),
    constraint team_activities_ibfk_2
        foreign key (email) references users(email),
    constraint team_activities_ibfk_3
        foreign key (sprint_num) references sprint_num_dates(sprint_num)
);

create table team_reports
(
    teams              varchar(50) not null ,
    sprint_num         int not null,
    understand_easiest varchar(300) null,
    understand_hardest varchar(300) null,
    approach_easiest   varchar(300) null,
    approach_hardest   varchar(300) null,
    solve_easiest      varchar(300) null,
    solve_hardest      varchar(300) null,
    evaluate_easiest   varchar(300) null,
    evaluate_hardest   varchar(300) null,
    completion         int null,
    contact            varchar(300) null,
    comments           varchar(300) null,
    primary key (teams, sprint_num),
    constraint team_report_ibfk_1
        foreign key (teams) references users(teams),
    constraint team_report_ibfk_2
        foreign key (sprint_num) references sprint_num_dates(sprint_num)
);

create table requirements
(
    teams       varchar(255) not null,
    indexs      int          not null,
    description varchar(255) null,
    primary key (indexs, teams),
    constraint requirements_ibfk_1
        foreign key (teams) references users(teams)
);

create table individual_reports
(
    email           varchar(255) not null,
    sprint_num         int          not null,
    monday_time        int          null,
    tuesday_time       int          null,
    wednesday_time     int          null,
    thursday_time      int          null,
    friday_time        int          null,
    saturday_time      int          null,
    sunday_time        int          null,
    discrepancy        varchar(300) null,
    request            varchar(300) null,

    primary key (email,sprint_num),
    constraint individual_report_ibfk_1
        foreign key (email) references users(email),
    constraint individual_report_ibfk_2
        foreign key (sprint_num) references sprint_num_dates(sprint_num)
);

