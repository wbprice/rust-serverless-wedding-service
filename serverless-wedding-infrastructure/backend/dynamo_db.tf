
locals {
    rsvp_table_id_index_name = "rsvp-id-index"
}

resource "aws_dynamodb_table" "rsvp_table" {
    name = "rsvp-${var.environment_code}-table"
    read_capacity = 10
    write_capacity = 10
    hash_key = "household_id"
    range_key = "name"
    stream_enabled = true
    stream_view_type = "NEW_IMAGE"

    global_secondary_index {
        name               = "${local.rsvp_table_id_index_name}"
        hash_key           = "id"
        write_capacity     = 5
        read_capacity      = 5
        projection_type    = "ALL"
    }

    attribute {
        name = "household_id"
        type = "S"
    }

    attribute {
        name = "name"
        type = "S"
    }

    attribute {
        name = "id"
        type = "S"
    }
}