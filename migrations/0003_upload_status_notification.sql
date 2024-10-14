-- First, let's create the function to notify status changes
CREATE OR REPLACE FUNCTION notify_upload_status()
RETURNS TRIGGER AS $$
BEGIN
    -- Use the processing_status column directly
    PERFORM pg_notify('upload_status', NEW.id || ',' || NEW.processing_status);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Now, let's create the trigger
CREATE TRIGGER upload_status_trigger
AFTER UPDATE OF processing_status ON videos
FOR EACH ROW
EXECUTE FUNCTION notify_upload_status();
