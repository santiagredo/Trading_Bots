export interface ErrorLog {
    id: number;
    creation_date: string;
    file_path: string;
    line_number: string;
    function_name: string;
    request: string;
    error_type: string;
    error_details: string;
}

export interface IntegrationLog {
    id: number;
    creation_date: string;
    integration_name: string;
    function_name: string;
    url: string;
    request: string;
    response: string;
    status_code: number;
    error_message: string;
    execution_time_ms: number;
}
