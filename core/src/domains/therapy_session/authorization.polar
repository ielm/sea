has_role(user: User, role_key: String, therapy_session: TherapySession) if 
    # therapy_session.therapist == user and
    # role_key == "therapist";
    role in user.roles and 
    role.role_key = role_key and
    role.resource_table = "therapy_session" and
    role.resource_id = therapy_session.id and
    therapy_session.therapist = user;


has_permission(_:User, "create", _: TherapySession);

resource TherapySession {
    permissions = [
        # Create a new session 
        "create", 
        # Update details about a session
        "update",
        # Delete a session
        "delete",
        # View a session
        "view",
        # Manage sessions
        "manage_sessions",
        # Grant or revoke profile roles for a session
        "manage_roles"
    ];

    roles = [
        # Clinician
        "clinician",
        # Client
        "client",
        # Admin
        "admin"
    ];

    "create" if "clinician";
    "update" if "clinician";
    "delete" if "clinician";
    "view" if "clinician";
    "manage_sessions" if "clinician";
    "manage_roles" if "clinician";

    "view" if "client";

    "manage_sessions" if "admin";
    "manage_roles" if "admin";
}